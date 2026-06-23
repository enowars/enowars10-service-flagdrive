import httpx
from logging import LoggerAdapter
from enochecker3 import Enochecker
import enochecker3.telemetry
from utils import FlagDriveClient

async def clean_request_hook(span, request):
    span.update_name(f"{request.method.decode()} {request.url}")
    if request.headers:
        for k, v in request.headers.items():
            span.set_attribute(f"http.headers.{k}", v)

async def clean_response_hook(span, request, response):
    pass

enochecker3.telemetry.async_request_hook = clean_request_hook
enochecker3.telemetry.async_response_hook = clean_response_hook

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


import importlib
import pkgutil

for module_name in ["exploit", "flagstores", "noise", "havoc"]:
    module = importlib.import_module(module_name)
    for _, name, _ in pkgutil.iter_modules(module.__path__):
        importlib.import_module(f"{module_name}.{name}")

if __name__ == "__main__":
    checker.run()