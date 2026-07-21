import json
import subprocess
from contextlib import redirect_stdout
from io import StringIO
from pathlib import Path


class Interpreter:
    def __init__(self, obj_path):
        # Guarda la ruta base del proyecto para resolver archivos relativos.
        self.oPath = Path(obj_path).resolve()

    def execute(self, obj: dict):
        # Obtiene los datos del objeto que viene desde el parser.
        command = obj.get("command")
        command_data = obj.get("command_data", {})
        args = obj.get("args", [])
        flags = obj.get("flags", [])
        tokens = obj.get("tokens", [])

        # Busca el archivo JSON donde están las rutas y funciones de los comandos.
        json_path = self.oPath / "lib" / "NaParse" / "command_path.json"
        try:
            with open(json_path, "r", encoding="utf-8") as osas:
                data = json.load(osas)
        except FileNotFoundError:
            return f"[bold green]No se encontró el archivo de rutas de comandos en {json_path}[/]"

        # Extrae los mapas de rutas y funciones desde el JSON.
        paths = data.get("Path", {})
        functions = data.get("Function", {})

        # Verifica que el comando exista en ambos mapas.
        if command not in paths or command not in functions:
            return f"[bold green]The given object ({command}) doesn't exists, verify if exists or you have permission to use it[/]"

        # Resuelve la ruta del módulo asociado al comando.
        module_path = self.oPath / paths[command]
        if not module_path.exists():
            return f"[bold green]The module for ({command}) was not found: {module_path}[/]"

        # Prepara un contexto simple con args, flags y tokens para que el comando pueda usarlos.
        context = {
            "args": args,
            "flags": flags,
            "tokens": tokens,
            "command": command,
            "command_data": command_data,
            "path": args[0] if args else "",
        }

        # Lee el contenido del módulo y lo ejecuta en un namespace aislado.
        module_code = module_path.read_text(encoding="utf-8")
        namespace = {"__name__": "__main__"}
        try:
            exec(compile(module_code, str(module_path), "exec"), namespace)
        except Exception as exc:
            return f"[bold red]Error loading module for {command}: {exc}[/]"

        # Formatea la función indicada en el JSON con los valores del contexto.
        function_code = functions[command]
        if "{path}" in function_code and args:
            function_code = function_code.format(path=args[0])
        elif "{" in function_code and "}" in function_code:
            try:
                function_code = function_code.format(**context)
            except KeyError:
                pass

        # Captura la salida estándar para devolverla como resultado del comando.
        output_buffer = StringIO()
        try:
            with redirect_stdout(output_buffer):
                exec(function_code, namespace)
        except SystemExit as exc:
            return f"exit({exc.code})"
        except Exception as exc:
            # Si no se pudo ejecutar como código Python, intenta como comando del shell.
            try:
                completed = subprocess.run(
                    function_code,
                    shell=True,
                    capture_output=True,
                    text=True,
                    cwd=self.oPath,
                    check=False,
                )
                output = completed.stdout.strip() or completed.stderr.strip()
                return output if output else ""
            except Exception as shell_exc:
                return f"[bold red]Error executing {command}: {exc}[/]"

        # Devuelve la salida capturada si hubo alguna; si no, devuelve una cadena vacía.
        output = output_buffer.getvalue().strip()
        return output if output else ""