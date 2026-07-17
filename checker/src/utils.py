import json
import httpx
from logging import LoggerAdapter
from enochecker3 import MumbleException

class FlagDriveClient:
    def __init__(self, http_client: httpx.AsyncClient, logger: LoggerAdapter):
        self.http_client = http_client
        self.logger = logger

    async def get_service_info(self) -> bytes:
        response = await self.http_client.get("/api/health")
        if response.status_code != 200:
            self.logger.error(f"Heartbeat failed: {response.text}")
            raise MumbleException("Heartbeat failed.")
        return response.content

    async def register_user(self, username: str, password: str) -> str:
        self.logger.info(f"Register new user: {username} with password: {password}")
        payload = {
            "username": username,
            "password": password
        }
        response = await self.http_client.post("/api/auth/register", json=payload)
        if response.status_code != 201:
            self.logger.error(f"Registration failed: {response.text}")
            raise MumbleException("Failed to register user")
        data = response.json()
        token = data.get("token")
        if not token:
            raise MumbleException("Token missing from registration response")

        self.logger.info(f"TOKEN: {token}")
        return token

    async def login_user(self, username: str, password: str) -> str:
        self.logger.info(f"Logging in user: {username} with password: {password}")
        payload = {
            "username": username,
            "password": password
        }
        response = await self.http_client.post("/api/auth/login", json=payload)
        if response.status_code != 200:
            self.logger.error(f"Login failed: {response.text}")
            raise MumbleException("Failed to log in")
        data = response.json()
        token = data.get("token")
        if not token:
            raise MumbleException("Token missing from login response")

        self.logger.info(f"TOKEN: {token}")
        return token

    async def verify_token(self, token: str) -> str:
        self.logger.info(f"Verifying token: {token}")
        payload = {"token": token}
        response = await self.http_client.post("/api/token/verify", json=payload)
        if response.status_code != 200:
            self.logger.error(f"Token verification failed: {response.text}")
            raise MumbleException("Failed to verify token")
        data = response.json()
        username = data.get("username")
        if not username:
            raise MumbleException("Username missing from token verification response")
        return username

    async def logout_token(self, token: str) -> None:
        self.logger.info(f"Logging out token: {token}")
        payload = {"token": token}
        response = await self.http_client.post("/api/token/logout", json=payload)
        if response.status_code != 200:
            self.logger.error(f"Logout failed: {response.text}")
            raise MumbleException("Failed to logout token")

    async def get_user_info(self, username: str) -> dict:
        self.logger.info(f"Getting user info for: {username}")
        response = await self.http_client.get(f"/api/user/{username}")
        if response.status_code != 200:
            self.logger.error(f"Get user info failed: {response.text}")
            raise MumbleException("Failed to get user info")
        return response.json()

    async def follow_user(self, current_username: str, target_username: str, token: str) -> None:
        self.logger.info(f"User {current_username} is following {target_username} with token {token}.")
        payload = {
            "token": token,
            "username": target_username
        }
        response = await self.http_client.post(f"/api/user/{current_username}/follow", json=payload)
        if response.status_code != 200:
            self.logger.error(f"Follow request failed: {response.text}")
            raise MumbleException("Failed to follow user")

    async def unfollow_user(self, current_username: str, target_username: str, token: str) -> None:
        self.logger.info(f"Unfollowing user: {target_username} as {current_username}")
        payload = {
            "token": token,
            "username": target_username
        }
        response = await self.http_client.post(f"/api/user/{current_username}/unfollow", json=payload)
        if response.status_code != 200:
            self.logger.error(f"Unfollow request failed: {response.text}")
            raise MumbleException("Failed to unfollow user")

    async def get_followers(self, username: str) -> list:
        self.logger.info(f"Getting followers list for: {username}")
        response = await self.http_client.get(f"/api/user/{username}/followers")
        if response.status_code != 200:
            self.logger.error(f"Get followers failed: {response.text}")
            raise MumbleException("Failed to get followers list")
        return response.json().get("followers", [])

    async def get_following(self, username: str) -> list:
        self.logger.info(f"Getting following list for: {username}")
        response = await self.http_client.get(f"/api/user/{username}/following")
        if response.status_code != 200:
            self.logger.error(f"Get following failed: {response.text}")
            raise MumbleException("Failed to get following list")
        return response.json().get("following", [])

    async def get_file_list(self, username: str, token: str = "") -> list:
        self.logger.info(f"Getting file list for user: {username}")
        payload = {}
        if token:
            payload["token"] = token
        response = await self.http_client.post(f"/api/files/{username}", json=payload)
        if response.status_code != 200:
            self.logger.error(f"Get file list failed: {response.text}")
            raise MumbleException("Failed to get file list")
        return response.json()

    async def upload_file(
        self,
        token: str,
        filename: str,
        file_content: bytes,
        encryption_key: str,
        visibility: int | str,
        backup: bool = False
    ) -> str:
        self.logger.info(f"Uploading file: {filename} with visibility: {visibility} (backup: {backup})")
        self.logger.info(f"File password: {encryption_key}")
        
        if isinstance(visibility, int):
            visibility_map = {
                0: "Private",
                1: "Public",
                2: "Following",
                3: "Followers",
            }
            visibility = visibility_map.get(visibility, "Private")

        metadata = {
            "token": token,
            "key": encryption_key,
            "visibility": visibility,
            "backup": backup
        }
        files = {
            "file": (filename, file_content, "application/octet-stream"),
            "json": (None, json.dumps(metadata), "application/json")
        }
        response = await self.http_client.post("/api/file/upload", files=files)
        if response.status_code != 201:
            self.logger.error(f"Upload failed: {response.text}")
            raise MumbleException("Failed to upload file")
        data = response.json()
        file_id = data.get("file_id")
        if file_id is None:
            raise MumbleException("File ID missing from upload response")
        return str(file_id)

    async def download_file(self, file_id: str, token: str, decryption_key: str, backup: bool = False) -> bytes:
        self.logger.info(f"Retrieving flag for file ID: {file_id} (backup: {backup})")
        payload = {
            "token": token,
            "key": decryption_key,
            "backup": backup
        }
        response = await self.http_client.post(f"/api/file/download/{file_id}", json=payload)
        if response.status_code != 200:
            self.logger.error(f"Download failed: {response.status_code} - {response.text}")
            raise MumbleException("Failed to download file")
        return response.content

    async def request_gdpr(self, token: str) -> str:
        self.logger.info(f"Requesting GDPR export")
        payload = {
            "token": token
        }
        response = await self.http_client.post("/api/gdpr/request", json=payload)
        if response.status_code != 200:
            self.logger.error(f"GDPR request failed: {response.text}")
            raise MumbleException("Failed to request GDPR export")
        data = response.json()
        gdpr_id = data.get("gdpr_id")
        if not gdpr_id:
            raise MumbleException("GDPR ID missing from response")
        return gdpr_id

    async def download_gdpr(self, gdpr_id: str) -> bytes:
        self.logger.info(f"Downloading GDPR export: {gdpr_id}")
        response = await self.http_client.get(f"/api/gdpr/download/{gdpr_id}")
        if response.status_code != 200:
            self.logger.error(f"GDPR download failed: {response.text}")
            raise MumbleException("Failed to download GDPR export")
        return response.content


import random

GOVERNMENT_MEMES = [
    "Birds Are Not Real",
    "Area 51 Security Clearance Protocol",
    "Classified War Thunder Tank Blueprints",
    "CIA Glow-in-the-Dark Operations",
    "FBI Webcam Monitoring Agent",
    "Operation MKUltra Phase 2",
    "Department of Silly Walks Directive",
    "Obama's Secret Last Name",
    "Defcon 1 Microwave Popcorn Instructions",
    "Classified UFO Retrieval Log",
    "Majestic 12 Meeting Minutes",
    "Federal Tax Loophole Cheatcodes",
    "Chemtrail Dispersal Schedule",
    "Illuminati Membership Application",
    "Nothing Happened On Square",
    "Pentagon Alien Database Backups",
    "SCP Foundation Containment Breach",
    "Cheyenne Mountain Stargate Log",
    "NSA PRISM Surveillance Metadata",
    "IRS Audit Exemption Certificate",
    "Lizard People Underground Highway Map",
    "Watergate Tape Missing 18 Minutes Transcript",
    "Weather Balloon Swapped for UFO Report",
    "There is no war in Ba Sing Se",
    "Dai Li Brainwashing Program Guidelines",
    "SERN Time Travel Research Archive",
    "IBN 5100 Decoding Manual",
    "John Titor Time Machine Operating Instructions",
    "Aperture Science Companion Cube Disposal Protocol",
    "The Cake Is A Lie Investigation Report",
    "Umbrella Corporation T-Virus Elite Genetic Design Plan",
    "SEELE Human Instrumentality Project Implementation Plan",
    "Future Gadget Laboratory: Operation Steins;Gate",
    "FGL Secret Agent Passcode: El Psy Kongroo",
    "Speedwagon Foundation Coffin Recovery Report: DIO is Alive"
]

def get_random_meme() -> str:
    return random.choice(GOVERNMENT_MEMES)

