import sys
import shutil
import os

def main():
	
	if len(sys.argv) != 3:
		print("Missing argument: ", len(sys.argv), " found, 3 needed")
		exit(1)
	
	S = int(sys.argv[1])
	C = int(sys.argv[2])
	
	with open("net_config.yml", "w") as f:
		
		f.write(f"S: {S}\n")
		f.write(f"C: {C}\n\n")
		
		for i in range(S):
			f.write(f"server{i}:\n")
			f.write(f" ip: server{i}\n")
			f.write(f" port: 12345\n\n")
		
		for i in range(C):
			f.write(f"client{i}:\n")
			f.write(f" ip: client{i}\n")
			f.write(f" port: 12345\n\n")
		
		f.close()
		
	encoins_dirname = os.getcwd()[:-7]
	shutil.copy("net_config.yml", encoins_dirname+"client/encoins/")
	shutil.copy("net_config.yml", encoins_dirname+"server/encoins/")

if __name__ == '__main__':
    main()
