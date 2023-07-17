#
# Copyright (c) 2022 ZettaScale Technology
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
#

from zenoh_flow.interfaces import Operator
from zenoh_flow import Inputs, Outputs
from zenoh_flow.types import Context
from typing import Dict, Any


class GreetingsMaker(Operator):
    def __init__(
        self,
        context: Context,
        configuration: Dict[str, Any],
        inputs: Inputs,
        outputs: Outputs,
    ):
        print(f"Context: {context}")
        self.output = outputs.take("greeting", str, lambda s: bytes(s, "utf-8"))
        self.in_stream = inputs.take("name", str, lambda buf: buf.decode("utf-8"))

        if self.in_stream is None:
            raise ValueError("No input 'name' found")
        if self.output is None:
            raise ValueError("No output 'greeting' found")

    def finalize(self) -> None:
        return None

    async def iteration(self) -> None:
        message = await self.in_stream.recv()
        name = message.get_data()
        if name is not None:
            greetings = self.generate_greetings(name)
            await self.output.send(greetings)

        return None

    def generate_greetings(self, name: str) -> str:
        greetings_dict = {
            "Sofia": "Ciao, {}!\n",
            "Leonardo": "Ciao, {}!\n",
            "Lucia": "¡Hola, {}!\n",
            "Martin": "¡Hola, {}!\n",
            "Jade": "Bonjour, {}!\n",
            "Gabriele": "Ciao, PaaS manager!\n",
        }

        greet = greetings_dict.get(name, "Hello, {}!\n")
        return greet.format(name)


def register():
    return GreetingsMaker
