def main() -> None:
    number_of_magnets: int = int(input())
    number_of_groups: int = 0
    current_magnet: str = ""

    for _ in range(number_of_magnets):
        magnet = input()
        if magnet != current_magnet:
            current_magnet = magnet
            number_of_groups += 1

    print(number_of_groups)

if __name__ == "__main__":
    main()