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



# class Node:
#     def initialize(configuration : dict ) -> State:
#         pass

#     def finalize( state: State) -> None:
#         pass


# class Operator:
#     def input_rule(ctx: Context, state: State, tokens: dict[PortId, Token]) -> bool:
#         pass

#     def run(ctx: Context, state: State, inputs: dict[PortId, DataMessage]) -> dict[PortId, Data]:
#         pass

#     def output_rule(ctx : Context, state: State, outputs: dict[PortId, Data]) -> dict[PortId, Data]:
#         pass


# class Sink:
#     def run(ctx: Context, state: State, input: DataMessage) -> None:
#         pass

# class Source:
#     def run(ctx: Context, state: State) -> Data:
#         pass


# class MyState:
#     def __init__(self, configuration):
#         self.value = 0
#         if configuration['value'] is not None:
#             self.value = int(configuration['value'])

# class MyPySource(Node, Operator):
#     def initialize(self, configuration):
#         return MyState(configuration)

#     def finalize(self, state):
#         return None

#     def run(self, _ctx, state):
#         state.value += 1
#         time.sleep(1)
#         return Data(state.value)


# def register():
#     return "MyPySource"