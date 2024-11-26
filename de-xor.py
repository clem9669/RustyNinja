import argparse

def xor_file(file_path, xor_value):
    # Open the file in binary read mode
    with open(file_path, 'rb') as file:
        content = file.read()

    # Convert the XOR value from hex to integer
    xor_value = int(xor_value, 16)

    # Apply the XOR operation to each byte in the content
    xored_content = bytes([byte ^ xor_value for byte in content])

    # Write the XORed content to a new file with the .out extension
    with open(f"{file_path}.out", 'wb') as output_file:
        output_file.write(xored_content)

if __name__ == "__main__":
    # Create an argument parser
    parser = argparse.ArgumentParser(description="XOR a file with a given hex value.")

    # Add arguments for file path and XOR value
    parser.add_argument('file_path', type=str, help="Path to the file to be XORed.")
    parser.add_argument('xor_value', type=str, help="Hex value to XOR the file with.")

    # Parse the command-line arguments
    args = parser.parse_args()

    # Run the XOR function with the provided arguments
    xor_file(args.file_path, args.xor_value)
