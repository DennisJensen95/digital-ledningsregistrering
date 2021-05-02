pipeline {
    agent { label 'master' } 
    stages {
        stage ("Build") {
            parallel {
                stage ("Build DJenzen frontend") {
                    steps {
                        sh 'docker-compose build frontend'
                    }
                }
                stage ("Build ler_2_database") {
                    steps {
                        sh 'docker-compose build ler_2_database'
                    }
                }
                stage ("Build ler_2_broker") {
                    steps {
                        sh 'docker-compose build ler_2_broker'
                    }
                }
            }
        }
    }
}