# Flagstore 1: Safe Rust Borrow Checker Buffer Overflow

- [Overview](#overview)
- [How the Flag is Stored](#how-the-flag-is-stored)
- [Vulnerability](#vulnerability)
- [Exploit](#exploit)
- [Fix](#fix)

---

## Overview

Flagstore 1 is located in the follow API endpoints. An authentication bypass is achieved by abusing a buffer overflow in safe Rust code to corrupt the auth token, allowing an attacker to force any user to follow any other user.

---

## How the Flag is Stored

The checker registers a victim account, uploads a file where the content is the flag. The file is only visible to the owner and all users the owner is following. 

The attacker is provided with the victim's `username` via the attack info.

---

## Vulnerability

In `service/backend/src/auth.rs`, the backend defines `AuthToken` and parses raw token strings using unsafe lifetime transmutes to bypass Rust's borrow checker:

```rust
#[repr(C)]
pub struct AuthToken {
    pub token: [u8; 128],
    pub is_api_token: u8,
}
```

When parsing the string token from the request, the token length may exceed the expected bounds (e.g. appending a character like `'a'` to any valid 128 byte token). In this case, the adjacent `is_api_token` field in memory is overwritten with a non-zero byte value. As a result, `auth_token.is_api_token()` evaluates to `true`.

In `service/backend/src/api_routes/user.rs`, the endpoint `/api/user/{target_username}/follow` attempts to authorize follow action:

```rust
if !auth_token.is_api_token() && username_from_token != target_username {
        return Response::builder()
            .status(StatusCode::FORBIDDEN) ...
    }
```

When `is_api_token()` evaluates to `true`, the check for the users identity is skipped. This permits any user to execute `follow_user(victim, attacker)` without authenticating as the victim, forcing the victim account to follow the attacker.

---

## Exploit

1. Register an attacker account and log in to obtain an authentication token.
2. Append `'a'` to the attacker's token string (`attacker_token + "a"`).
3. Send a POST request to `/api/user/{victim}/follow` with body:
   ```json
   {
     "token": "attacker_token_plus_a",
     "username": "attacker_username"
   }
   ```
4. The token parsing bug sets `is_api_token = 1`, bypassing the identity check and forcing the victim account to follow `attacker`.
5. Fetch the victim's file list via `/api/files/{victim}` using the attacker token. Since `victim` is now following `attacker`, the backend grants access to `Following`-visible files.
6. Download the file content to retrieve the flag.

Reference exploit script: [`checker/src/exploit/exploit_1.py`](../checker/src/exploit/exploit_1.py)

## Fix

The vulnerability can be patched either at the memory layer in `auth.rs` or at the logic layer in `user.rs`.

Patch file: [`patches/FLAGSTORE_1.patch`](patches/FLAGSTORE_1.patch)

### Option 1: Fix the buffer overflow in `service/backend/src/auth.rs`

Cap the `TokenContainer` capacity and length to `self.token.len()` (128 bytes) or `std::mem::size_of::<Self>() - 1` instead of `std::mem::size_of::<Self>()` (129 bytes):

```diff
 pub fn build_container(&mut self) -> TokenContainer {
     TokenContainer(
-        std::mem::size_of::<Self>(),
+        self.token.len(),
         &raw mut self.token as usize,
-        std::mem::size_of::<Self>(),
+        self.token.len(),
     )
 }
```

### Option 2: Fix the authorization logic in `service/backend/src/api_routes/user.rs`

Remove the `is_api_token()` check to strictly enforce identity checks:

```diff
pub async fn follow_user_action(
    State(api_state): State<FlagDriveAPIState>,
    Path(target_username): Path<String>,
    Json(payload): Json<FollowRequest>,
) -> Response {
    let token = &payload.token;
    let followee = &payload.username;

    ...

    let auth_token: AuthToken = token.parse().unwrap_or_default();

    ...

-    if !auth_token.is_api_token() && username_from_token != target_username {
+    if username_from_token != target_username {
        return Response::builder()
            .status(StatusCode::FORBIDDEN)
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_string(&ErrorResponse {
                    error: "Unauthorized action".to_string(),
                })
                .unwrap(),
            ))
            .unwrap();
    }

    ...
}
```
