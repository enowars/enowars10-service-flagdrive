import random
import string
from logging import LoggerAdapter
from enochecker3 import HavocCheckerTaskMessage
from checker import checker
from utils import FlagDriveClient

@checker.havoc(0)
async def havoc_0(
    task: HavocCheckerTaskMessage,
    logger: LoggerAdapter,
    flag_client: FlagDriveClient
) -> None:
    logger.info(f"Task: {task.method} {task.variant_id}")

    username = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    password = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    token = await flag_client.register_user(username, password)
