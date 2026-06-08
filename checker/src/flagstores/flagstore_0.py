import random
import string
import json
from logging import LoggerAdapter
from enochecker3 import ChainDB, PutflagCheckerTaskMessage, GetflagCheckerTaskMessage, MumbleException
from checker import checker
from utils import FlagDriveClient

@checker.putflag(0)
async def putflag_gdpr_info(
    task: PutflagCheckerTaskMessage,
    db: ChainDB,
    flag_client: FlagDriveClient,
    logger: LoggerAdapter
) -> str:
    logger.info(f"Task: {task.method} {task.variant_id}")
    username = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    password = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    token = await flag_client.register_user(username, password)

    encryption_key = "".join(random.choices(string.ascii_letters + string.digits, k=32))
    file_id = await flag_client.upload_file(
        token,
        f"{task.flag}",
        b"El Psy Kongroo",
        encryption_key,
        0,
    )
    logger.info(f"File uploaded, file ID: {file_id}")
    
    gdpr_id = await flag_client.request_gdpr(token)
    logger.info(f"GDPR requested, received GDPR ID: {gdpr_id}")

    await db.set("userdata", (gdpr_id))

    return username

@checker.getflag(0)
async def getflag_gdpr_info(
    task: GetflagCheckerTaskMessage,
    db: ChainDB,
    logger: LoggerAdapter,
    flag_client: FlagDriveClient
) -> None:
    logger.info(f"Task: {task.method} {task.variant_id}")

    try:
        gdpr_id = await db.get("userdata")
    except KeyError:
        logger.info("Missing database entry from putflag")
        raise MumbleException("Missing database entry from putflag")

    content = await flag_client.download_gdpr(gdpr_id)
    try:
        data = json.loads(content.decode())
    except Exception as e:
        logger.info("Failed to parse GDPR data as JSON")
        raise MumbleException("GDPR data is not valid JSON")

    files_list = data.get("files")
    if not isinstance(files_list, list):
        logger.info(f"GDPR files field is not a list: {files_list}")
        raise MumbleException("GDPR data files field is invalid")
            
    logger.info(f"File list in GDPR data: {files_list}")
    if task.flag not in files_list:
        logger.info(f"Flag '{task.flag}' not found in GDPR data files: {files_list}")
        raise MumbleException("Flag not found in GDPR data files")
