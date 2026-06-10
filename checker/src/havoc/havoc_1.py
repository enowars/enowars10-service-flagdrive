import random
import string
from logging import LoggerAdapter
from enochecker3 import HavocCheckerTaskMessage, MumbleException
from checker import checker
from utils import FlagDriveClient

@checker.havoc(1)
async def havoc_1(
    task: HavocCheckerTaskMessage,
    logger: LoggerAdapter,
    flag_client: FlagDriveClient
) -> None:
    logger.info(f"Task: {task.method} {task.variant_id}")

    username = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    password = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    token = await flag_client.register_user(username, password)

    filename = "".join(random.choices(string.ascii_lowercase, k=8)) + ".txt"
    file_content = "".join(random.choices(string.ascii_letters, k=64)).encode()
    encryption_key = "".join(random.choices(string.ascii_letters, k=16))
    
    file_id = await flag_client.upload_file(
        token=token,
        filename=filename,
        file_content=file_content,
        encryption_key=encryption_key,
        visibility=0
    )

    downloaded_content = await flag_client.download_file(
        file_id=file_id,
        token=token,
        decryption_key=encryption_key
    )
    if downloaded_content != file_content:
        raise MumbleException("Downloaded file content mismatch")

    file_list = await flag_client.get_file_list(username, token)
    if not any(str(f.get("id")) == str(file_id) for f in file_list):
        raise MumbleException("File not found in file list")
