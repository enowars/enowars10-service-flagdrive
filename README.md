# FlagDrive (ENOWARS10 Service)

FlagDrive is a secure, cloud-based government file storage and sharing portal developed for the **ENOWARS10 "Hack the Government" Attack/Defense CTF**. It allows citizens and government officials to upload documents, manage visibility clearances, securely download encrypted/unencrypted files, and follow/unfollow other users.

---

## Subproject Documentation

For detailed guides on how to install, build, run, and develop the different parts of FlagDrive, refer to their individual READMEs:

* **[Frontend SPA (Leptos + Tailwind CSS) Documentation](service/frontend/README.md)**
* **[Backend Web Server (Axum + SQLx + SQLite) Documentation](service/backend/README.md)**

---

## Running the Full Stack (Docker)

The fastest and most reliable way to spin up the entire FlagDrive stack is using Docker Compose. This starts both the Axum backend and automatically serves the pre-compiled WebAssembly frontend.

### Prerequisites
* [Docker](https://docs.docker.com/get-docker/)
* [Docker Compose](https://docs.docker.com/compose/install/)

### Commands

**1. Launch the service:**
```bash
cd service
sudo docker compose up -d --build
```
This builds the chef-optimized multi-stage Docker image and deploys FlagDrive.

**2. Accessing the application:**
Once the containers are running, navigate your browser to:
* **Frontend Portal & Backend API:** `http://127.0.0.1:4859`

**3. Stop the service:**
```bash
sudo docker compose down
```

---

## License

This project is licensed under the [MIT License](LICENSE).