pipeline {
    agent { label 'master' } 
    stages {
        stage ("Build applications") {
            steps {
                sh 'docker-compose build --parallel'
            }
        }
    }
}