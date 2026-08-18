"""Da parser for Natrix, idk maybe i should try Luau or Scratch"""
# maybe its too complicated to develop a parser but idk, i think
# da main class
import json
import re
from .interpreter import Interpreter
# from core import NEC
class _Token:
    def __init__(self, token_type, value):
        self.type = token_type
        self.value = value

    _QUOTED = re.compile(r'"(?P<dq>[^"]*)"|\'(?P<sq>[^\']*)\'|(?P<word>\S+)')

    @staticmethod
    def _tokenize(text: str):
        tokens = []
        for match in _Token._QUOTED.finditer(text):
            quoted = match.group("dq")
            if quoted is None:
                quoted = match.group("sq")
            if quoted is not None:
                tokens.append(_Token("WORD", quoted))
                continue
            word = match.group("word")
            if word.startswith("--"):
                tokens.append(_Token("FLAG", word))
            elif word.startswith("-") and len(word) > 1:
                tokens.append(_Token("SHORT_FLAG", word))
            else:
                tokens.append(_Token("WORD", word))
        return tokens
from pathlib import Path
class Parser:
    def __init__(self, commands_path=None):
        if commands_path is None:
            commands_path = Path(__file__).resolve().parent / "commands.json"
        self.commands_path = Path(commands_path)
        self.tokens = []
    def load_commands(self):
        with open(self.commands_path, "r", encoding="utf-8") as cdm:
            data = json.load(cdm)
        return data.get("commands", [])
    def parse(self, text: str):
        self.tokens = _Token._tokenize(text)
        commands = self.load_commands()

        if not self.tokens:
            raise ValueError("No text for parse")

        command_token = self.tokens[0]
        if command_token.type != "WORD":
            raise ValueError("Se esperaba un comando (WORD) como primer token")

        command_name = command_token.value
        command = None
        for cmd in commands:
            if command_name == cmd["name"] or command_name in cmd.get("aliases", []):
                command = cmd
                break

        if command is None:
            return (f"Comando desconocido: {command_name}")

        args = []
        flags = []
        for token in self.tokens[1:]:
            if token.type == "WORD":
                args.append(token.value)
            else:
                flags.append(token.value)

        dataArray={
            "command": command_name,
            "command_data": command,
            "args": args,
            "flags": flags,
            "tokens": self.tokens}
        print(dataArray)
        interpreter = Interpreter(Path(__file__).resolve().parent.parent)
        interpreter.execute(obj=dataArray)
    def _get_next_word(self, index: int):
        next_index = index + 1
        if next_index < len(self.tokens):
            next_token = self.tokens[next_index]
            if next_token.type == "WORD":
                return next_token.value
        return None
if __name__ == "__main__":
    debug=False
    if debug==True:
        Parser.parse(debug=True)
    elif debug==False:
        pass
    else:
        print("Unknown Option...")
        exit(1)
    print("'parser.py' is a library/module, isn't made to run like a normal Python (.py) file, please use it on a script...")
    exit(0)