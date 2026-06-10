import random
import string
from logging import LoggerAdapter
from enochecker3 import HavocCheckerTaskMessage, MumbleException
from checker import checker
from utils import FlagDriveClient

@checker.havoc(3)
async def havoc_3(
    task: HavocCheckerTaskMessage,
    logger: LoggerAdapter,
    flag_client: FlagDriveClient
) -> None:
    logger.info(f"Task: {task.method} {task.variant_id}")

    username = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    password = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    token = await flag_client.register_user(username, password)

    await flag_client.logout_token(token)
    
    try:
        await flag_client.verify_token(token)
        raise MumbleException("Token is still valid after logout")
    except MumbleException as e:
        if "Token is still valid" in str(e):
            raise
        pass
