import random
import string
from logging import LoggerAdapter
from enochecker3 import HavocCheckerTaskMessage, MumbleException
from checker import checker
from utils import FlagDriveClient

@checker.havoc(2)
async def havoc_2(
    task: HavocCheckerTaskMessage,
    logger: LoggerAdapter,
    flag_client: FlagDriveClient
) -> None:
    logger.info(f"Task: {task.method} {task.variant_id}")

    username = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    password = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    token = await flag_client.register_user(username, password)

    gdpr_id = await flag_client.request_gdpr(token)

    parts = gdpr_id.split("-")
    if len(parts) >= 3:
        latest_gdpr_id = f"{parts[0]}-latest-{parts[2]}"
        await flag_client.download_gdpr(latest_gdpr_id)

    await flag_client.download_gdpr(gdpr_id)
