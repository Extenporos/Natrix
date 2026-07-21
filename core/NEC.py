"""# Natrix Engine Core
Is the core. what are you expecting??"""
# Natrix Engine Core, do not share all or partially the file...
# Under Development, some things can change
# Import
try:
    from rich.layout import Layout
    from rich.table import Table
    from rich.console import Console
    from pathlib import Path
    import socket, os
    import sys, time
    import json
    from lib.NaLog.logs import Loggins
except ImportError:
    print("'logs.py' is missing or modified, please download the file of the source code from GitHub (https://github.com/Extenporos/Natrix) and move it to Natrix/core...")
    exit(1)
class NECFunctions:
    def __init__(self):
        self.console=Console()
        pass
class Registry:
    def __init__(self):
        # ruta base del registro; no crear ni pedir username en el init -Copilot
        # gracias por avisarme Copilot, no me avisas we -VortexNN
        self.regPath = Path.home() / ".natrix" / ".config" / "reg"
        self.defaultUser = self.regPath / "Users" / "defaultUser"

    def makeRegistry(self):
        """
        Crea la estructura de directorios y archivos por defecto.
        Devuelve True si se creó/existe correctamente, False en error.
        """
        try:
            # crear estructura
            self.defaultUser.mkdir(parents=True, exist_ok=True)

            # solicitar username solo si no existe el archivo
            username_file = self.defaultUser / "username.txt"
            id_file = self.defaultUser / "userID.txt"
            if not username_file.exists():
                username = self.makeUsername(action=1) or "default"
                username_file.write_text(username, encoding="utf-8")
            if not id_file.exists():
                hostname = socket.gethostname()
                id_file.write_text(hostname, encoding="utf-8")
            return True
        except Exception as e:
            # logs.safe_log(...) si tienes un mecanismo de logs
            return False

    def verifyIfTheres2Users(self):
        """
        Devuelve (loginTrigger: bool, users: dict) siempre.
        """
        usersDir = self.regPath / "Users"
        if not usersDir.exists():
            return False, {}
        users = {
            folder.name: folder
            for folder in usersDir.iterdir()
            if folder.is_dir()
        }
        loginTrigger = len(users) >= 2
        return loginTrigger, users

    def getID(self):
        user_id_path = self.defaultUser / "userID.txt"
        if user_id_path.exists():
            return user_id_path.read_text(encoding="utf-8").strip()
        return None

    def getUser(self):
        username_path = self.defaultUser / "username.txt"
        if username_path.exists():
            return username_path.read_text(encoding="utf-8").strip()
        return None

    def makeUsername(self, action=0):
        """
        action==1: pide y devuelve un username por input.
        """
        if action == 1:
            try:
                username = input("Type your username to save it on the Registry...\n>> ").strip()
                return username
            except Exception:
                return None
        return None

    def manageRegistry(self):
        pass

# this is bc i dont want to some guy try to run this file alone, and say "why this file cant be run alone?"
if __name__ == "__main__":
    print("'NEC.py' is a module of Natrix Environment Core, it can't run alone.\nPlease run './Natrix' or 'python Natrix.py' to run the program...")
    sys.exit(1)