from asyncio import StreamReader, StreamWriter
import asyncio
import random
import string
import faker


from typing import Optional
from logging import LoggerAdapter

from enochecker3 import (
    ChainDB,
    Enochecker,
    ExploitCheckerTaskMessage,
    FlagSearcher,
    PutflagCheckerTaskMessage,
    GetflagCheckerTaskMessage,
    PutnoiseCheckerTaskMessage,
    GetnoiseCheckerTaskMessage,
    HavocCheckerTaskMessage,
    MumbleException,
    OfflineException,
    InternalErrorException,
    PutflagCheckerTaskMessage,
    AsyncSocket,
)
from enochecker3.utils import assert_equals, assert_in

import re
import json
import httpx

"""
Checker config
"""

SERVICE_PORT = 4859
checker = Enochecker("flagdrive", SERVICE_PORT)
app = lambda: checker.app


"""
Utility functions
"""

class FlagDriveClient:
    def __init__(self, http_client: httpx.AsyncClient, logger: LoggerAdapter):
        self.http_client = http_client
        self.logger = logger

    async def register_user(self, username: str, password: str) -> str:
        self.logger.info(f"Registering user: {username}")
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
        return token

    async def login_user(self, username: str, password: str) -> str:
        self.logger.info(f"Logging in user: {username}")
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
        return token

    async def upload_file(
        self,
        token: str,
        filename: str,
        file_content: bytes,
        encryption_key: str,
        visibility: int
    ) -> int:
        self.logger.info(f"Uploading file: {filename} with visibility: {visibility}")
        metadata = {
            "token": token,
            "encryption_key": encryption_key,
            "visibility": visibility
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
        return file_id

    async def download_file(self, file_id: int, token: str, decryption_key: str) -> bytes:
        self.logger.info(f"Retrieving flag for file ID: {file_id}")
        payload = {
            "token": token,
            "decryption_key": decryption_key
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


@checker.register_dependency
def _get_flag_client(http_client: httpx.AsyncClient, logger: LoggerAdapter) -> FlagDriveClient:
    return FlagDriveClient(http_client, logger)


"""
CHECKER FUNCTIONS
"""

@checker.putflag(0)
async def putflag_gdpr_info(
    task: PutflagCheckerTaskMessage,
    db: ChainDB,
    flag_client: FlagDriveClient,
    logger: LoggerAdapter
) -> None:
    username: str = "".join(
        random.choices(string.ascii_uppercase + string.digits, k=12)
    )
    password: str = "".join(
        random.choices(string.ascii_uppercase + string.digits, k=12)
    )

    logger.info(f'Register new user:{username} with password:{password}')
    token = await flag_client.register_user(username, password)
    logger.info(f'TOKEN: {token}')

    logger.info(f'Upload encrypted private file with file name: {task.flag}')
    filename = f"{task.flag}"
    encryption_key = "".join(
        random.choices(string.ascii_letters + string.digits, k=32)
    )

    file_id = await flag_client.upload_file(
        token,
        filename,
        b"El Psy Kongroo",
        encryption_key,
        0,
    )
    logger.info(f"File uploaded, file ID: {file_id}")
    
    logger.info("Requesting GDPR export...")
    gdpr_id = await flag_client.request_gdpr(token)
    logger.info(f"GDPR requested, received GDPR ID: {gdpr_id}")

    await db.set("userdata", (gdpr_id))

    return username

@checker.getflag(0)
async def getflag_gdpr_info(
    task: GetflagCheckerTaskMessage,
    db: ChainDB,
    flag_client: FlagDriveClient,
    logger: LoggerAdapter
) -> None:
    try:
        gdpr_id = await db.get("userdata")
    except KeyError:
        logger.info("Missing database entry from putflag")
        raise MumbleException("Missing database entry from putflag")

    content = await flag_client.download_gdpr(gdpr_id)
    
    try:
        data = json.loads(content.decode())
    except Exception as e:
        logger.info(f"Failed to parse GDPR export as JSON: {e}")
        raise MumbleException("GDPR export is not valid JSON")
        
    files_list = data.get("files")
    if not isinstance(files_list, list):
        logger.info(f"GDPR files field is not a list: {files_list}")
        raise MumbleException("GDPR export files field is invalid")
            
    logger.info(f"Files list in GDPR export: {files_list}")
    if task.flag not in files_list:
        logger.info(f"Flag '{task.flag}' not found in GDPR export files: {files_list}")
        raise MumbleException("Flag not found in GDPR export files list")

@checker.putnoise(0)
async def putnoise_gdpr_info(
    task: PutnoiseCheckerTaskMessage,
    db: ChainDB,
    flag_client: FlagDriveClient,
    logger: LoggerAdapter
) -> None:
    pass

@checker.getnoise(0)
async def getnoise_gdpr_info(
    task: GetnoiseCheckerTaskMessage,
    db: ChainDB,
    flag_client: FlagDriveClient,
    logger: LoggerAdapter
) -> None:
    pass

@checker.exploit(0)
async def exploit_gdpr_info(
    task: ExploitCheckerTaskMessage,
    searcher: FlagSearcher,
    flag_client: FlagDriveClient,
    logger: LoggerAdapter
) -> Optional[str]:
    logger.info(f"Attacker info: {task.attack_info}")

    content = b""
    for i in range(16):
        try:
            content = await flag_client.download_gdpr(f"{task.attack_info}-latest-{hex(i)[2:]}%")
            break
        except Exception:
            pass

    try:
        data = json.loads(content.decode())
    except Exception as e:
        logger.info(f"Failed to parse GDPR export as JSON: {e}")
        raise MumbleException("GDPR export is not valid JSON")
        
    files_list = data.get("files")
    if not isinstance(files_list, list):
        logger.info(f"GDPR files field is not a list: {files_list}")
        raise MumbleException("GDPR export files field is invalid")

    logger.info(f"Files list in GDPR export: {files_list}")
    
    for file in files_list:
        if searcher.search_flag(file) != None:
            return file

    return None

# @checker.putflag(1)
# async def putflag_encrypted_file(
#     task: PutflagCheckerTaskMessage,
#     db: ChainDB,
#     flag_client: FlagDriveClient,
#     logger: LoggerAdapter
# ) -> None:
#     username: str = "".join(
#         random.choices(string.ascii_uppercase + string.digits, k=12)
#     )
#     password: str = "".join(
#         random.choices(string.ascii_uppercase + string.digits, k=12)
#     )
    
#     token = await flag_client.register_user(username, password)
#     filename = f"flag_{username}.txt"
#     encryption_key = "".join(
#         random.choices(string.ascii_letters + string.digits, k=32)
#     )
    
#     file_id = await flag_client.upload_file(
#         token,
#         filename,
#         task.flag.encode(),
#         encryption_key,
#         0,  # Private visibility
#     )
    
#     await db.set("userdata", {
#         "username": username,
#         "password": password,
#         "token": token,
#         "filename": filename,
#         "file_id": file_id,
#         "encryption_key": encryption_key
#     })
    
#     return f"File ID: {file_id}"


# @checker.getflag(1)
# async def getflag_encrypted_file(
#     task: GetflagCheckerTaskMessage,
#     db: ChainDB,
#     flag_client: FlagDriveClient,
#     logger: LoggerAdapter
# ) -> None:
#     try:
#         userdata = await db.get("userdata")
#     except KeyError:
#         raise MumbleException("Missing database entry from putflag")
        
#     file_id = userdata["file_id"]
#     token = userdata["token"]
#     encryption_key = userdata["encryption_key"]
    
#     content = await flag_client.download_file(file_id, token, encryption_key)
    
#     assert_equals(
#         task.flag.encode(),
#         content,
#         message="Flag mismatch"
#     )


# @checker.putflag(2)
# async def putflag_bypass_auth(
#     task: PutflagCheckerTaskMessage,
#     db: ChainDB,
#     flag_client: FlagDriveClient,
#     logger: LoggerAdapter
# ) -> None:
#     username: str = "".join(
#         random.choices(string.ascii_uppercase + string.digits, k=12)
#     )
#     password: str = "".join(
#         random.choices(string.ascii_uppercase + string.digits, k=12)
#     )
    
#     token = await flag_client.register_user(username, password)
#     filename = f"flag_{username}.txt"
#     encryption_key = "".join(
#         random.choices(string.ascii_letters + string.digits, k=32)
#     )
    
#     file_id = await flag_client.upload_file(
#         token,
#         filename,
#         task.flag.encode(),
#         encryption_key,
#         3,  # Followers visibility
#     )
    
#     await db.set("userdata", {
#         "username": username,
#         "password": password,
#         "token": token,
#         "filename": filename,
#         "file_id": file_id,
#         "encryption_key": encryption_key
#     })
    
#     return f"File ID: {file_id}"


# @checker.getflag(2)
# async def getflag_bypass_auth(
#     task: GetflagCheckerTaskMessage,
#     db: ChainDB,
#     flag_client: FlagDriveClient,
#     logger: LoggerAdapter
# ) -> None:
#     try:
#         userdata = await db.get("userdata")
#     except KeyError:
#         raise MumbleException("Missing database entry from putflag")
        
#     file_id = userdata["file_id"]
#     token = userdata["token"]
#     encryption_key = userdata["encryption_key"]
    
#     content = await flag_client.download_file(file_id, token, encryption_key)
    
#     assert_equals(
#         task.flag.encode(),
#         content,
#         message="Flag mismatch"
#     )

# @checker.exploit(1)
# async def exploit_encrypted_file(
#     task: ExploitCheckerTaskMessage,
#     searcher: FlagSearcher,
#     flag_client: FlagDriveClient,
#     logger: LoggerAdapter
# ) -> Optional[str]:
#     return None

# @checker.exploit(2)
# async def exploit_bypass_auth(
#     task: ExploitCheckerTaskMessage,
#     searcher: FlagSearcher,
#     flag_client: FlagDriveClient,
#     logger: LoggerAdapter
# ) -> Optional[str]:
#     return None


# @checker.putnoise(1)
# async def putnoise_encrypted_file(
#     task: PutnoiseCheckerTaskMessage,
#     db: ChainDB,
#     flag_client: FlagDriveClient,
#     logger: LoggerAdapter
# ) -> None:
#     pass

# @checker.getnoise(1)
# async def getnoise_encrypted_file(
#     task: GetnoiseCheckerTaskMessage,
#     db: ChainDB,
#     flag_client: FlagDriveClient,
#     logger: LoggerAdapter
# ) -> None:
#     pass

# @checker.putnoise(2)
# async def putnoise_bypass_auth(
#     task: PutnoiseCheckerTaskMessage,
#     db: ChainDB,
#     flag_client: FlagDriveClient,
#     logger: LoggerAdapter
# ) -> None:
#     pass

# @checker.getnoise(2)
# async def getnoise_bypass_auth(
#     task: GetnoiseCheckerTaskMessage,
#     db: ChainDB,
#     flag_client: FlagDriveClient,
#     logger: LoggerAdapter
# ) -> None:
#     pass

# @checker.havoc(0)
# async def havoc_gdpr_info(
#     task: HavocCheckerTaskMessage,
#     flag_client: FlagDriveClient,
#     logger: LoggerAdapter
# ) -> None:
#     pass

# @checker.havoc(1)
# async def havoc_encrypted_file(
#     task: HavocCheckerTaskMessage,
#     flag_client: FlagDriveClient,
#     logger: LoggerAdapter
# ) -> None:
#     pass

# @checker.havoc(2)
# async def havoc_bypass_auth(
#     task: HavocCheckerTaskMessage,
#     flag_client: FlagDriveClient,
#     logger: LoggerAdapter
# ) -> None:
#     pass


if __name__ == "__main__":
    checker.run()