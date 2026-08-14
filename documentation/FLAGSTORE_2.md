# Flagstore 2: AES-256-GCM IV Reuse & Authentication Tag Forgery

- [Overview](#overview)
- [How the Flag is Stored](#how-the-flag-is-stored)
- [Vulnerability](#vulnerability)
- [Exploit](#exploit)
- [Fix](#fix)

---

## Overview

Flagstore 2 is located in the file backup endpoints used to export and import file backups. Due to the deterministically generated per-user IV, reusing the same key and IV allows an attacker to solve for the GHASH key $H$ and secret mask $S$ over $GF(2^{128})$. This enables the attacker to forge a valid GCM authentication tag and import the victim's ciphertext into their own account to retrieve the decrypted flag.


---

## How the Flag is Stored

The checker registers a victim user, sets an passphrase, and uploads a file encrypted with AES-256-GCM with `Public` visibility. The ciphertext contains the flag.

The attacker is provided with the victim's `username` via CTF attack info.

---

## Vulnerability

In `service/backend/src/crypto.rs`, IVs for AES-256-GCM encryption are derived deterministically using a static hash of the username (`construct_iv(username)`):

```rust
pub fn construct_iv(username: &str) -> [u8; 12] {
    let hash = Sha256::digest(username.as_bytes());
    let mut iv = [0u8; 12];
    iv.copy_from_slice(&hash[..12]);
    iv
}
```

Because every file under a given username uses the exact same 12-byte IV under the same AES key $K$, the AES-GCM security is guaranteed to be broken.

In Galois Counter Mode (GCM), operations are performed over the finite field $GF(2^{128})$ defined by the irreducible polynomial:
$$f(x) = x^{128} + x^7 + x^2 + x + 1$$

The authentication tag $T$ is calculated as:
$$T = \text{GHASH}_H(A, C) \oplus S$$

Where:
- $H = \text{AES}_K(0^{128})$ is the GHASH key.
- $S = \text{AES}_K(\text{IV} \parallel 1)$ is the secret mask derived from the IV.
- $A$ is the Additional Authenticated Data (AAD)
- $C$ is the ciphertext.

By collecting multiple ciphertexts and tags generated under the same key/IV pair, an attacker cancels out the secret mask $S$ ($T_1 \oplus T_2 = \Delta \text{GHASH}_H$) and sets up polynomial equations in $GF(2^{128})[H]$:
$$P(H) = \Delta \text{GHASH}_H(A, C) \oplus \Delta T = 0$$

Using polynomial root finding (GCD over $GF(2^{128})[H]$), the attacker recovers $H$, calculates $S = T_1 \oplus \text{GHASH}_H(A_1, C_1)$, and can now compute a valid GCM authentication tag for any ciphertext bound to any AAD.

The backend upload endpoint (`/api/files/upload` with `backup = true`) accepts raw `IV || ciphertext || tag bytes` and attempts to decrypt them using `aes_gcm_decrypt_no_verify` after verifying the provided tag.

The GHASH algorithm is defined in [AES-GCM](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38d.pdf).

---

## Exploit

1. Download the victim's public encrypted file bytes (`IV || ciphertext || tag`).
2. Register an attacker account and upload 2+ dummy files.
3. Download the dummy files to obtain four `(IV, ciphertext_i, tag_i)` pairs.
4. Construct polynomials over $GF(2^{128})$ using GHASH definitions to extract the GHASH authentication key $H$.
5. Compute the secret mask $S$ and forge a valid GCM authentication tag for the victim's ciphertext bound to `target_aad = victim_iv + attacker_username`.
6. Upload the forged `IV || ciphertext || forged_tag` backup to the attacker's account using the `backup` upload parameter.
7. Download the file from the attacker account. The server verifies the backup and returns the decrypted flag.

Reference exploit script: [`checker/src/exploit/exploit_2.py`](../checker/src/exploit/exploit_2.py)

---

## Fix

The primary and intended fix is to enforce that the `provided_iv` during a backup upload strictly matches the authenticated account's derived IV (`constructed_iv`). This prevents an attacker from uploading ciphertext encrypted under a victim's IV and re-encryption under their own account.

Patch file: [`patches/FLAGSTORE_2.patch`](patches/FLAGSTORE_2.patch)

In `service/backend/src/api_routes/files.rs`:

```diff
        let mut provided_iv = [0u8; 12];
        provided_iv.copy_from_slice(&content_bytes[0..12]);
        let ct_tag = &content_bytes[12..];

        let constructed_iv = construct_iv(&username);

+        if provided_iv != constructed_iv {
+            return Response::builder()
+                .status(StatusCode::BAD_REQUEST)
+                .header("content-type", "application/json")
+                .body(Body::from(
+                    serde_json::to_string(&ErrorResponse {
+                        error: "Invalid backup IV".to_string(),
+                    })
+                    .unwrap(),
+                ))
+                .unwrap();
+        }
```

---

### Alternative Patching Strategies & MUMBLE Risks

Other fixes exist, but **naïvely implementing them will cause a `MUMBLE` status** in the EnoChecker:

1. **Random IV Generation per Upload**:
   Replacing `construct_iv(username)` with `generate_random_iv()`. This IV need to be stored somewhere.
2. **Per-File Deterministic IV or Key Derivation**:
   Deriving IVs or keys using `(username, file_id)` ensures unique IVs per file and breaks GHASH tag forgery.

