import argparse
import os

def xor_file(file_path, xor_value):
    try:
        # Check if the file exists
        if not os.path.isfile(file_path):
            print(f"Error: The file '{file_path}' does not exist.")
            return
        
        # Open the file in binary read mode
        with open(file_path, 'rb') as file:
            content = file.read()

        # Convert the XOR value from hex to integer
        xor_value = int(xor_value, 16)

        # Apply the XOR operation to each byte in the content
        xored_content = bytes([byte ^ xor_value for byte in content])

        # Define output file name
        output_file_path = f"{file_path}.out"

        # Write the XORed content to a new file with the .out extension
        with open(output_file_path, 'wb') as output_file:
            output_file.write(xored_content)

        # Output success information
        print(f"Success: The XOR-ed file has been written to '{output_file_path}'.")
        print(f"Original file size: {len(content)} bytes")
        print(f"XOR-ed file size: {len(xored_content)} bytes")

    except ValueError:
        print("Error: Invalid XOR value. Please provide a valid hexadecimal value.")
    except IOError as e:
        print(f"Error: An I/O error occurred: {e}")

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
