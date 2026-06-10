import random
import string
from logging import LoggerAdapter
from enochecker3 import ChainDB, PutnoiseCheckerTaskMessage, GetnoiseCheckerTaskMessage, MumbleException
from checker import checker
from utils import FlagDriveClient

@checker.putnoise(1)
async def putnoise_1(
    task: PutnoiseCheckerTaskMessage,
    db: ChainDB,
    logger: LoggerAdapter,
    flag_client: FlagDriveClient
) -> None:
    logger.info(f"Task: {task.method} {task.variant_id}")

    username: str = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    password: str = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    token = await flag_client.register_user(username, password)

    gdpr_id = await flag_client.request_gdpr(token)

    await db.set("userdata", (username, password, token, gdpr_id))

@checker.getnoise(1)
async def getnoise_1(
    task: GetnoiseCheckerTaskMessage,
    db: ChainDB,
    logger: LoggerAdapter,
    flag_client: FlagDriveClient
) -> None:
    logger.info(f"Task: {task.method} {task.variant_id}")

    try:
        username, password, token, gdpr_id = await db.get("userdata")
    except KeyError:
        logger.info("Missing database entry from putnoise")
        raise MumbleException("Missing database entry from putnoise")

    parts = gdpr_id.split("-")
    if len(parts) >= 3:
        latest_gdpr_id = f"{parts[0]}-latest-{parts[2]}"
        await flag_client.download_gdpr(latest_gdpr_id)
        
    await flag_client.download_gdpr(gdpr_id)
