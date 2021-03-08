case $1 in
	deploy)
		echo "Building deploy Docker"
		docker build -t backend/develop $2 ./develop
		docker build -t backend/deploy $2 ./deploy
		;;
	develop)
		echo "Building develop Docker"
		docker build -t backend/develop_2 $2 ./develop
		;;
	*)
		echo "Please input deploy or develop"
		;;
  esac