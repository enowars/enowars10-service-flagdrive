import random
import string
from logging import LoggerAdapter
from enochecker3 import ChainDB, PutnoiseCheckerTaskMessage, GetnoiseCheckerTaskMessage, MumbleException
from checker import checker
from utils import FlagDriveClient

@checker.putnoise(2)
async def putnoise_2(
    task: PutnoiseCheckerTaskMessage,
    db: ChainDB,
    logger: LoggerAdapter,
    flag_client: FlagDriveClient
) -> None:
    logger.info(f"Task: {task.method} {task.variant_id}")

    user1 = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    pass1 = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    token1 = await flag_client.register_user(user1, pass1)

    user2 = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    pass2 = "".join(random.choices(string.ascii_uppercase + string.digits, k=12))
    token2 = await flag_client.register_user(user2, pass2)

    await flag_client.follow_user(user1, user2, token1)
    await db.set("userdata", (user1, pass1, token1, user2, pass2, token2))

@checker.getnoise(2)
async def getnoise_2(
    task: GetnoiseCheckerTaskMessage,
    db: ChainDB,
    logger: LoggerAdapter,
    flag_client: FlagDriveClient
) -> None:
    logger.info(f"Task: {task.method} {task.variant_id}")

    try:
        user1, pass1, token1, user2, pass2, token2 = await db.get("userdata")
    except KeyError:
        raise MumbleException("Missing database entry from putnoise")

    following1 = await flag_client.get_following(user1)
    if user2 not in following1:
        raise MumbleException("User1 is not following User2")

    followers2 = await flag_client.get_followers(user2)
    if user1 not in followers2:
        raise MumbleException("User2 follower list missing User1")

    await flag_client.unfollow_user(user1, user2, token1)

    following1_after = await flag_client.get_following(user1)
    if user2 in following1_after:
        raise MumbleException("User1 is still following User2 after unfollow")

    await flag_client.follow_user(user1, user2, token1)
