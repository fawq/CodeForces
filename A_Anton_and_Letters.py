def main() -> None:
    set_of_letters: str = input()
    set_of_letters = set_of_letters.translate({ord(letter): None for letter in '{},'})

    letters: set[int] = set(set_of_letters.split())

    print(len(letters))

if __name__ == "__main__":
    main()