# Use the official Node.js 18 image based on Alpine for a minimal footprint
FROM node:18-alpine

# Set the working directory inside the container
WORKDIR /app

# Copy package.json and package-lock.json
COPY package*.json ./

# Install dependencies
RUN npm ci --production

# Copy the rest of the application code to the container
COPY . .

# Expose the port that the Express app listens on (default is 3000)
EXPOSE 3000

# Start the application
CMD ["node", "index.js"]
