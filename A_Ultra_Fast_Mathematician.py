def main() -> None:
    bits_str: str = input()
    bits2_str: str = input()

    for bit1, bit2 in zip(bits_str, bits2_str):
        if bit1 != bit2:
            print("1", end="")
        else:
            print("0", end="")

if __name__ == "__main__":
    main()