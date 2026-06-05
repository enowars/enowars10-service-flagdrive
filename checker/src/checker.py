import httpx
from logging import LoggerAdapter
from enochecker3 import Enochecker
from utils import FlagDriveClient

"""
Checker config
"""

SERVICE_PORT = 4859
checker = Enochecker("flagdrive", SERVICE_PORT)
app = lambda: checker.app


@checker.register_dependency
def _get_flag_client(http_client: httpx.AsyncClient, logger: LoggerAdapter) -> FlagDriveClient:
    return FlagDriveClient(http_client, logger)


"""
CHECKER DIAGNOSTIC
"""

@checker.test(0)
async def test_0(
    logger: LoggerAdapter,
    flag_client: FlagDriveClient
) -> None:
    content = await flag_client.get_service_info()
    logger.info(f"Service info: {content}")


import exploit
from flagstores import flagstore_0, flagstore_1
from noise import noise_0
from havoc import havoc_0

if __name__ == "__main__":
    checker.run()