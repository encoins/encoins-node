import yaml
import sys

def main():

	if len(sys.argv) != 2:
		print("Missing argument: ", len(sys.argv), " found, 2 needed")
		exit(1)
		
	# Read the architecture file
	
	with open(sys.argv[1], "r") as archi_file:
	
		lines = archi_file.read()
		archi_file.close()

	archi = yaml.load(lines, Loader=yaml.FullLoader)
	
	# Write the docker-compose file
	
	with open("docker-compose.yml", "w") as f:
	
		f.write("version: \"3.3\"\n\n")
		f.write("services:\n\n")
		
		for i in range(archi["S"]):
			f.write(f" server{i}:\n")
			f.write("  build: ../server/encoins\n")
			f.write(f"  container_name: server{i}\n")
			f.write(f"  environment:\n   - NUM_NODE={i}\n\n")
			
		for i in range(archi["C"]):
			f.write(f" client{i}:\n")
			f.write("  build: ../client/encoins\n")
			f.write(f"  container_name: client{i}\n")
			f.write(f"  depends_on:\n   - server{i}\n")
			f.write(f"  environment:\n   - NUM_NODE={i}\n\n")
			
		f.close()

if __name__ == '__main__':
    main()
