import random
import string
from logging import LoggerAdapter
from enochecker3 import HavocCheckerTaskMessage, MumbleException
from checker import checker
from utils import FlagDriveClient

@checker.havoc(5)
async def havoc_5(
    task: HavocCheckerTaskMessage,
    logger: LoggerAdapter,
    flag_client: FlagDriveClient
) -> None:
    logger.info(f"Task: {task.method} {task.variant_id}")

    user1 = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    pass1 = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    token1 = await flag_client.register_user(user1, pass1)

    filename = "".join(random.choices(string.ascii_lowercase, k=8)) + ".txt"
    file_content = "".join(random.choices(string.ascii_letters, k=64)).encode()
    encryption_key = "".join(random.choices(string.ascii_letters, k=16))

    file_id = await flag_client.upload_file(
        token=token1,
        filename=filename,
        file_content=file_content,
        encryption_key=encryption_key,
        visibility=1
    )

    user2 = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    pass2 = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    token2 = await flag_client.register_user(user2, pass2)

    file_list = await flag_client.get_file_list(user1, token2)
    
    if not any(str(f.get("id")) == str(file_id) for f in file_list):
        raise MumbleException("Public file not found in other user's file list")

    downloaded_content = await flag_client.download_file(
        file_id=file_id,
        token=token2,
        decryption_key=encryption_key
    )

    if downloaded_content != file_content:
        raise MumbleException("Downloaded public file content mismatch")
