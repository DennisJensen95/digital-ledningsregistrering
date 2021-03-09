case $1 in
	deploy)
		echo "Building deploy Docker"
		docker build -t backend/develop -f Docker/develop/Dockerfile $2 .
		docker build -t backend/deploy -f Docker/deploy/Dockerfile $2 .
		;;
	develop)
		echo "Building develop Docker"
		docker build -t backend/develop -f Docker/develop/Dockerfile $2 .
		;;
	*)
		echo "Please input deploy or develop"
		;;
  esac