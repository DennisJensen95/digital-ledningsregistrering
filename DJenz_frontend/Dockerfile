# pull official base image
FROM node:15.13.0-alpine

# set working directory
WORKDIR /app

# add `/app/node_modules/.bin` to $PATH
ENV PATH /app/node_modules/.bin:$PATH

# install app dependencies
# add app
COPY . ./
RUN npm install --silent
RUN npm run build



# start app
CMD ["npm", "run", "start"]