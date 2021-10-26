import time

class MyState:
    def __init__(self, configuration):
        self.value = 0
        if configuration['value'] is not None:
            self.value = int(configuration['value'])


class Node:
    def initialize(self, configuration):
        return MyState(configuration)

    def finalize(self, state):
        return None


class PySource:
    def run(self, _ctx, state):
        state.value += 1
        time.sleep(1)
        return state.value