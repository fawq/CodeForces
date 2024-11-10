def main() -> None:
    dubstep: str = input()
    dubstep = dubstep.replace("WUB", " ").strip()
    dubstep = dubstep.replace("  ", " ")

    print(dubstep)

if __name__ == "__main__":
    main()