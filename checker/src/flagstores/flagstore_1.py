import random
import string
from logging import LoggerAdapter
from enochecker3 import ChainDB, PutflagCheckerTaskMessage, GetflagCheckerTaskMessage, MumbleException
from checker import checker
from utils import FlagDriveClient

@checker.putflag(1)
async def putflag_follow_download(
    task: PutflagCheckerTaskMessage,
    db: ChainDB,
    flag_client: FlagDriveClient,
    logger: LoggerAdapter
) -> str:
    logger.info(f"Task: {task.method} {task.variant_id}")
    username: str = "".join(
        random.choices(string.ascii_uppercase + string.digits, k=12)
    )
    password: str = "".join(
        random.choices(string.ascii_uppercase + string.digits, k=12)
    )
    token = await flag_client.register_user(username, password)

    file_id = await flag_client.upload_file(
        token,
        "El Psy Kongroo",
        task.flag.encode(),
        "",
        2,
    )
    logger.info(f"File uploaded, file ID: {file_id}")

    await db.set("userdata", (token, str(file_id)))

    return username

@checker.getflag(1)
async def getflag_follow_download(
    task: GetflagCheckerTaskMessage,
    db: ChainDB,
    logger: LoggerAdapter,
    flag_client: FlagDriveClient
) -> None:
    logger.info(f"Task: {task.method} {task.variant_id}")

    try:
        (token, file_id) = await db.get("userdata")
    except KeyError:
        logger.info("Missing database entry from putflag")
        raise MumbleException("Missing database entry from putflag")

    content = await flag_client.download_file(int(file_id), token, "")
    if task.flag.encode() not in content:
        logger.info(f"Flag '{task.flag}' not found in downloaded file.")
        raise MumbleException("Flag not found in downloaded file")
