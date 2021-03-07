case $1 in
	deploy)
		echo "Building deploy Docker"
		docker build -t backend/develop ./develop
		docker build -t backend/deploy ./deploy
		;;
	develop)
		echo "Building develop Docker"
		docker build -t backend/develop ./develop
		;;
	*)
		echo "Please input deploy or develop"
		;;
  esac