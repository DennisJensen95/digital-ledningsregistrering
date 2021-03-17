FROM ubuntu:18.04

RUN apt update && apt dist-upgrade -y
RUN apt install software-properties-common -y

RUN add-apt-repository ppa:alex-p/qcad
RUN apt-get update
RUN apt-get install qcad -y

COPY qcad-3.26.1-trial-linux-x86_64.run /
RUN cd / && chmod +x qcad-3.26.1-trial-linux-x86_64.run
RUN cd / && ./qcad-3.26.1-trial-linux-x86_64.run
WORKDIR /root/opt/qcad-3.26.1-trial-linux-x86_64
