def main() -> None:
    word: str = input()
    hello_word: str = "hello"
    index: int = 0

    for character in word:
        if character == hello_word[index]:
            index += 1
        if index == 5:
            print("YES")
            break
    else:
        print("NO")

if __name__ == "__main__":
    main()