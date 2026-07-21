# Comments in Spanish
# aki haciendo kosas
#import
from rich.progress import (
    Progress,
    BarColumn,
    DownloadColumn,
    TransferSpeedColumn,
    TimeRemainingColumn,
    )
from rich.console import Console
from rich.table import Table
import requests
import hashlib
import zipfile
#import subprocess
import os
import shutil
from pathlib import Path
# Descargar el programa y ver su Hash
class Download:
    def __init__(self):
        self.console = Console()
        self.cli()
        pass
    def cli(self):
        default_dir=Path.home() / ".natrix"
        self.console.print("[bold italic bright_blue]Welcome to the Natrix Installer! Follow me on Tiktok and YouTube as @VortexNDot[/]")
        self.path = input(f"Please enter the path where you want to install Natrix (default: {default_dir}): ")
        if not self.path or "":
            self.path=default_dir

    def download_and_parse_manifest(self):
        try:
            manifest = requests.get('https://raw.githubusercontent.com/Extenporos/Natrix/main/manifest.json').json()
        except requests.JSONDecodeError:
            self.console.print("[bold bright_red]The file 'manifest.json' doesn't exists on the Natrix repository, contact VortexNN in Discord or in the repository make an issue, please try again...[/]")
            exit()
        self.version = manifest["version"]
        self.hash = manifest["sha256"]
        self.size = manifest["size"]
        self.filename = manifest["assets"]["name"]
        self.origin = manifest.get("assets", {}).get("origin", "")
        
    def download_file(self):
        URL='https://github.com/Extenporos/Natrix/releases/latest'
        r=requests.get(URL, stream=True)
        return r

    def check_hash(self):
        self.sha = hashlib.sha256()
        Corrupt=None
        with open(self.filename, "rb") as f:
            while chunk := f.read(1024 * 1024):
                self.sha.update(chunk)
        hash2 = self.sha.hexdigest()
        hash1 = self.hash
        if hash1 == hash2:
            Corrupt=False
            return Corrupt
        else:
            Corrupt=True
            self.console.print("[bold bright_blue]The downloaded version of Natrix is [bold italic red]corrupted[/] or [bold underline yellow]modified[/], please select what to do...[/]")
            c=input(">> ")
            if c.lower() == "del":
                os.remove(self.filename)
                exit(1)                                           
            elif c.lower() == "exit":
                exit(1)
            else:
                os.remove(self.filename)
                exit(1)
    
    def main(self):
        self.download_and_parse_manifest()
        total = int(self.download_file().headers.get('content-length', 0))
        r=self.download_file()
        with Progress(
            "[progress.description]{task.description}",
            BarColumn(),
            "[progress.percentage]{task.percentage:>3.0f}%",
            DownloadColumn(),
            TransferSpeedColumn(),
            TimeRemainingColumn(),
        ) as progress:
            task = progress.add_task("[bright_green]Downloading...", total=total)
            self.download_file()
            with open(self.filename, "wb") as f:
                for chunk in r.iter_content(chunk_size=8192):
                    f.write(chunk)
                    progress.update(task, advance=len(chunk))
        self.check_hash()
        z = zipfile.ZipFile(self.filename, 'r')
        os.mkdir(self.path)
        z.extractall(self.path, self.filename)
        z.close()
        os.remove(self.filename)
        self.console.print("[green]Natrix has been downloaded and installed successfully, enjoy it![/green]")
        exit()

class menu:
    def __init__(self):
        self.console = Console()
        self.mainmenu(status=0)
        pass
    def mainmenu(self, status):
        table = Table(title="[bold bright_cyan]Natrix Setup[/]")
        self.console.print("[bold bright_blue]Welcome to the Natrix Setup, please select what you want to do...[/]")
        table.add_row("1", "[bold bright_green]Install Natrix[/]")
        table.add_row("2", "[bold bright_red]Uninstall Natrix[/]")
        table.add_row("3", "[bold bright_cyan]Exit Natrix Setup[/]")
        self.console.print(table)
        while status == 0:
            c2 = input("$ ").lower()
            if c2 == "1":
                Download().main()
            elif c2 == "2":
                home = Path.home()
                found = False
                for folder in home.rglob(".natrix"):
                    if folder.is_dir():
                        found = True
                        shutil.rmtree(folder)
                        self.console.print(f"[bold italic green]Succesfully deleted Natrix in: [/] {folder}")
                        break
                if not found:
                    self.console.print("[yellow]No Natrix instalation was found.[/yellow]")

            elif c2 == "3":
                status = 1
            else:
                self.console.print("[red]Invalid option.[/red]")

if __name__ == "__main__":
    menu()