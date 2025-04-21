from collections import Counter

def main() -> None:
    first_word: str = input()
    second_word: str = input()
    third_word: str = input()

    if Counter(first_word) + Counter(second_word) == Counter(third_word):
        print("YES")
    else:
        print("NO")

if __name__ == "__main__":
    main()