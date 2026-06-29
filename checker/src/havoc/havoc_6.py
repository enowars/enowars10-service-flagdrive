import random
import string
from logging import LoggerAdapter
from enochecker3 import HavocCheckerTaskMessage, MumbleException
from checker import checker
from utils import FlagDriveClient

@checker.havoc(6)
async def havoc_6(
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
        visibility=0,
        backup=False
    )

    backup_content = await flag_client.download_file(
        file_id=file_id,
        token=token,
        decryption_key="",
        backup=True
    )

    if len(backup_content) < 28:
        raise MumbleException("Backup content is too short")

    backup_filename = "".join(random.choices(string.ascii_lowercase, k=8)) + ".txt"

    new_file_id = await flag_client.upload_file(
        token=token,
        filename=backup_filename,
        file_content=backup_content,
        encryption_key=encryption_key,
        visibility=0,
        backup=True
    )

    restored_content = await flag_client.download_file(
        file_id=new_file_id,
        token=token,
        decryption_key=encryption_key,
        backup=False
    )

    if restored_content != file_content:
        raise MumbleException("Restored file content does not match original file content")
