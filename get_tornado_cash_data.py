import requests
import json
from os import environ
from itertools import count
from time import sleep, time
from sys import exit as sys_exit, argv, stderr


API_URL = "https://api.etherscan.io/api"
ACCOUNTS = {
    "TORNADO_CASH_0_1ETH": "0x12D66f87A04A9E220743712cE6d9bB1B5616B8Fc",
    "TORNADO_CASH_1ETH": "0x47CE0C6eD5B0Ce3d3A51fdb1C52DC66a7c3c2936",
    "TORNADO_CASH_10ETH": "0x910Cbd523D972eb0a6f4cAe4618aD62622b39DbF",
    "TORNADO_CASH_100ETH": "0xA160cdAB225685dA1d56aa342Ad8841c3b53f291",
    "TORNADO_CASH_ROUTER": "0xd90e2f925DA726b50C4Ed8D0Fb90Ad053324F31b",
}


def api_call(api_key: str, module: str, action: str, **kwargs):
    return requests.get(
        f"{API_URL}?module={module}&action={action}&" +
        f"{'&'.join(f'{k}={v}' for (k, v) in kwargs.items())}&apikey={api_key}"
    ).json()


def account(api_key: str, action: str, **kwargs):
    return api_call(api_key, "account", action, **kwargs)


def block_number(api_key: str):
    return int(
        api_call(api_key, "proxy", "eth_blockNumber")["result"],
        base=16
    )


def account_transactions(
    api_key: str,
    address: str,
    startblock: int,
    endblock: int,
    page: int,
    offset: int = 1000,
    sort: str = "asc"
):
    return account(
        api_key,
        "txlist",
        address=address,
        startblock=startblock,
        endblock=endblock,
        page=page,
        offset=offset,
        sort=sort
    )


def get_data(
    api_key: str,
    step_size: int,
    start_block: int,
    end_block: int,
    address: str
):
    history = []

    while True:
        t0 = time()
        response = account_transactions(
            api_key,
            address,
            start_block,
            end_block,
            0,
            offset=step_size
        )

        if not response["message"].startswith("OK"):
            print(response["message"], file=stderr)
            sys_exit(1)

        result = response["result"]
        history += result

        if len(result) < step_size or start_block == end_block:
            break

        start_block = int(result[-1]["blockNumber"])
        if api_key == "YourApiKeyToken":
            t1 = 6 - (time() - t0)
            sleep(t1 if t1 > 0 else 0)

    return history


def main():
    api_key = environ.get("ETHERSCAN_API_KEY")
    step_size = 10000
    start_block = 9117609
    end_block = block_number(api_key)

    for (account_name, addr) in ACCOUNTS.items():
        history = get_data(api_key, step_size, start_block, end_block, addr)

        with open(f"{account_name}.json", "w+") as fp:
            json.dump(history, fp)


if __name__ == "__main__":
    main()
