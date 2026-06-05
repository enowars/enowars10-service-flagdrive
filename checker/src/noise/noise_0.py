import random
import string
from logging import LoggerAdapter
from enochecker3 import ChainDB, PutnoiseCheckerTaskMessage, GetnoiseCheckerTaskMessage, MumbleException
from checker import checker
from utils import FlagDriveClient

@checker.putnoise(0)
async def putnoise_0(
    task: PutnoiseCheckerTaskMessage,
    db: ChainDB,
    logger: LoggerAdapter,
    flag_client: FlagDriveClient
) -> None:
    logger.info(f"Task: {task.method} {task.variant_id}")

    username: str = "".join(
        random.choices(string.ascii_uppercase + string.digits, k=12)
    )
    password: str = "".join(
        random.choices(string.ascii_uppercase + string.digits, k=12)
    )
    token = await flag_client.register_user(username, password)

    await db.set("userdata", (username, password, token))

@checker.getnoise(0)
async def getnoise_0(
    task: GetnoiseCheckerTaskMessage,
    db: ChainDB,
    logger: LoggerAdapter,
    flag_client: FlagDriveClient
) -> None:
    logger.info(f"Task: {task.method} {task.variant_id}")

    try:
        username, password, token = await db.get("userdata")
    except KeyError:
        logger.info("Missing database entry from putnoise")
        raise MumbleException("Missing database entry from putnoise")
