# Flagstore 0: GDPR Export Nonce Prefix Bruteforce

- [Overview](#overview)
- [How the Flag is Stored](#how-the-flag-is-stored)
- [Vulnerability](#vulnerability)
- [Exploit](#exploit)
- [Fix](#fix)

---

## Overview

Flagstore 0 is located in the GDPR export feature. An unauthorized attacker can download a target user's GDPR metadata export, which contains the filenames of all uploaded files, by guessing a single character in the nonce part of the URL.

---

## How the Flag is Stored

The checker registers a victim account, uploads a file where the filename is the flag, and triggers a GDPR export request via `/api/gdpr/request`. The resulting GDPR export now contains the flag inside a JSON file.

The attacker is provided with the victim's `username` via the attack info.

---

## Vulnerability

In `service/backend/src/api_routes/gdpr.rs` and `service/backend/src/database/gdpr.rs`, the export download endpoint `/api/gdpr/download/{gdpr_id}` accepts a parameter string formatted as `<username>-<timestamp>-<nonce>`.

When `timestamp` is specified as `"latest"`, the database layer executes the following query:

```sql
SELECT content FROM gdpr_data 
WHERE username = $1 AND nonce >= $2 AND nonce < $3 
ORDER BY timestamp DESC LIMIT 1;
```

The values for $2 and $3 are derived from the input `{gdpr_id}` and are used to do a prefix search on the nonce. Since hexadecimal characters span `0-9a-f` (16 total possibilities), an attacker only needs at most 16 HTTP GET requests to guess the prefix byte of the victim's GDPR nonce and download their export.

---

## Exploit

1. Read the target victim `username` from the attack info.
2. Send HTTP GET requests to `/api/gdpr/download/{username}-latest-{hex_char}` for `hex_char` in `["0", "1", ..., "f"]`.
3. Exactly one of the 16 requests will match the target nonce prefix and return the JSON export.
4. Extract the filename (flag) from the JSON response.

Reference exploit script: [`checker/src/exploit/exploit_0.py`](../checker/src/exploit/exploit_0.py)

---

## Fix

The fix enforces that the `nonce` length is exactly 32 characters long and that the SQL query uses an exact equality match instead of a prefix search.

Patch file: [`patches/FLAGSTORE_0.patch`](patches/FLAGSTORE_0.patch)

In `service/backend/src/database/gdpr.rs`:

```diff
 pub async fn get_gdpr_data(
     pool: &DbPool,
     username: &str,
     timestamp_or_latest: &str,
     nonce: &str,
 ) -> Result<String, sqlx::Error> {
-    let mut nonce_upper = nonce.to_string();
-    if let Some(last_char) = nonce_upper.pop() {
-        let next_char = (last_char as u8 + 1) as char;
-        nonce_upper.push(next_char);
+    if nonce.len() != 32 {
+        return Err(sqlx::Error::RowNotFound);
     }

     let row = if timestamp_or_latest == "latest" {
         sqlx::query(
-            "SELECT content FROM gdpr_data WHERE username = $1 AND nonce >= $2 AND nonce < $3 \
+            "SELECT content FROM gdpr_data WHERE username = $1 AND nonce = $2 \
              ORDER BY timestamp DESC LIMIT 1",
         )
         .bind(username)
         .bind(nonce)
-        .bind(&nonce_upper)
         .fetch_one(pool)
         .await?
     } else {
         let timestamp: i64 = timestamp_or_latest
             .parse()
             .map_err(|_| sqlx::Error::RowNotFound)?;

         sqlx::query(
-            "SELECT content FROM gdpr_data WHERE username = $1 AND timestamp = $2 AND nonce >= $3 AND nonce < $4",
+            "SELECT content FROM gdpr_data WHERE username = $1 AND timestamp = $2 AND nonce = $3",
         )
         .bind(username)
         .bind(timestamp)
         .bind(nonce)
-        .bind(&nonce_upper)
         .fetch_one(pool)
         .await?
     };
```
