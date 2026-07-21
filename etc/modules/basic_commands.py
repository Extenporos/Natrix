# comandos basicos, bastante simple
from rich.table import Table
from rich.layout import Layout
from rich.console import Console
import json
def helper():
    with open("etc/commands.json", "r") as bb:
        commands=list(json.load(bb)) # creo que para la descripcion, no se we
        description=list(json.load(bb))
    layout=Layout()
    layout.split_column(# lo divide en dos el helper
        Layout(name="header", size=3),# header
        Layout(name="body") # cuerpo del helper
    )
    layout["body"].split_row( # divide el cuerpo en dos para hacer "Command" y "Description"
        Layout(name="left"),
        Layout(name="right")
    )
    # tabla izquierda
    leftT=Table(title="Commands")
    leftT.add_column("Command")

    for i in range(len(commands)):
        leftT.add_row(commands[i])
    # Tabla derecha
    rightT=Table(title="Descriptions")
    rightT.add_column("Description")
    # hacer la tabla
    for ii in range(len(description)):
        rightT.add_row(description[ii])
    layout["left"].update(leftT) # actualizar la izquierda
    layout["right"].update(rightT) # lo mismo pero en la derecha

    Console.print(layout) # imprimir el helper