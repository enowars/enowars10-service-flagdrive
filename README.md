# FlagDrive (ENOWARS10 Service)

FlagDrive is a secure, government-hosted document storage and file-sharing portal developed for the **ENOWARS10 Attack/Defense CTF**. It allows citizens and government officials to upload and download encrypted documents with different access permissions, as well as follow or unfollow other users to share documents with them.

---

## Repository Branches

* **`main`**: Contains the vulnerable version of the FlagDrive service.
* **`fixed`**: Contains the patched version of the FlagDrive service.

---

## Quick Start

### Running the Service (Docker)

```bash
cd service
docker compose up -d --build
```

The application and API will be available at `http://127.0.0.1:4859`.

### Running the Checker (Docker)

```bash
cd checker
docker compose up -d --build
```

---

## Documentation

For full details on service architecture, flagstores, vulnerabilities, exploits, fixes, and component internals:

* **[CTF Service Overview](documentation/README.md)**
* **[Backend Architecture](documentation/BACKEND.md)**
* **[Frontend Architecture](documentation/FRONTEND.md)**
* **[Flagstore 0: GDPR Export Nonce Prefix Bruteforce](documentation/FLAGSTORE_0.md)**
* **[Flagstore 1: Safe Rust Borrow Checker Buffer Overflow](documentation/FLAGSTORE_1.md)**
* **[Flagstore 2: AES-256-GCM IV Reuse & GHASH Forgery](documentation/FLAGSTORE_2.md)**

---

## License

This project is licensed under the [MIT License](LICENSE).