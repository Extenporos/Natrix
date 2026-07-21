# Natrix CLI, idk what to put here
# in this moment the comments are in spanish
from rich.console import Console
from pathlib import Path
from core.NEC import Registry
from lib.NaParse.parser import Parser
class CLI:
    def __init__(self):
        self.console = Console()
        self.parser=Parser()
        # registrar referencia al Registry para usarla en todo el objeto
        self.reg=Registry()
        self.Terminal()
        
    def check_if_register_needs_setup(self):
        try:
            return bool(self.reg.makeRegistry())
        except Exception:
            return False
             
    def get_values_from_register(self, action=""):
        reg = self.reg
        if not reg:
            pass
        if action == "getID":
            ID=reg.getID()
            import random
            fallback="VortexMachine"
            posibillities=[fallback, "userMachine", "nobody", "we-dont-know", "anonimus", "imposter", "amogus"]
            if ID is not None:
                return ID
            elif ID is None:
                rts=random.choice(posibillities)
                return rts
        elif action == "getUser":
            import random
            USER=reg.getUser()
            fallback2="Tornado69"
            pososo=[fallback2, "unknown", "joemama", "also-i-dont-know", "hakar", "sus", "idk-wth"]
            if USER is not None:
                return USER
            elif USER is None:
                rs=random.choice(pososo)
                return rs
        elif action == "makeUsername":    
            try:
                return reg.makeUsername()
            except Exception:
                return None
        return None

    def Terminal(self):
        id_user = self.get_values_from_register("getID")
        user = self.get_values_from_register("getUser")
        pPath = Path.cwd()
        home=Path.home()
        if pPath.is_relative_to(home):
            actual_path="~/" + str(pPath.relative_to(home))
        else:
            actual_path=str(pPath)
        """Terminal Thing"""
        try:
            while True:
                raw = self.console.input(f"[bold bright_green]{user}@{id_user}[/]:[bold blue]{actual_path}[/]$ ").strip()
                if raw:
                    result = self.parser.parse(text=raw)
                    self.console.print(result)
                elif not raw:
                    continue
                else:
                    self.console.print(f"[bold cyan]Unknown command ({raw}), please try again...")
                    continue

        except Exception as e:
            self.console.print(f"\n[bold bright_yellow]Natrix Error:[bold red] An error ocurred while the execution: {e}[/]")
            
if __name__ == "__main__":
    CLI()
