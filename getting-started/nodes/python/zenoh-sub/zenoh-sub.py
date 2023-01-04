from zenoh_flow.interfaces import Sink
from zenoh_flow import DataReceiver
from zenoh_flow.types import Context
from typing import Dict, Any
import zenoh
from zenoh import Reliability
import asyncio
import json

DEFAULT_ZENOH_LOCATOR = "tcp/127.0.0.1:7447"
DEFAULT_MODE = "client"
DEFAULT_KE = "zf/getting-started/hello"
PERIOD = 0.5


class ZenohSub(Sink):
    def __init__(
        self,
        context: Context,
        configuration: Dict[str, Any],
        inputs: Dict[str, DataReceiver],
    ):
        self.output = inputs.get("out", None)
        if self.output is None:
            raise ValueError("Could not find output 'out'")

        self.locator = configuration.get("locator", DEFAULT_ZENOH_LOCATOR)
        self.mode = configuration.get("mode", DEFAULT_MODE)
        self.ke = configuration.get("key_expression", DEFAULT_KE)

        self.zconf = zenoh.Config()

        self.zconf.insert_json5(zenoh.config.MODE_KEY, json.dumps(self.mode))
        self.zconf.insert_json5(
            zenoh.config.CONNECT_KEY, json.dumps([self.locator])
        )

        self.session = zenoh.open(self.zconf)

        self.sub = self.session.declare_subscriber(
            self.ke,
            self.on_sensor_update,
            reliability=Reliability.RELIABLE(),
        )
        self.name = None

    def finalize(self):
        self.sub.undeclare()
        self.session.close()

    def on_sensor_update(self, sample):
        self.name = sample.payload

    async def iteration(self):
        await asyncio.sleep(PERIOD)

        if self.name is not None:
            await self.output.send(self.name)
            self.name = None

        return None


def register():
    return ZenohSub
