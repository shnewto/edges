def display(dir: int) -> str:
    def display(expr) -> str:
        return "1" if expr else "_"

    return (
        f"{display(dir & 1)}{display(dir & 64)}{display(dir & 2)}\n"
        f"{display(dir & 16)}1{display(dir & 32)}\n"
        f"{display(dir & 4)}{display(dir & 128)}{display(dir & 8)}"
    )
