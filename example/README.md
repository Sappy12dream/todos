// example.ts

// Define an interface for a User
interface User {
    id: number;
    name: string;
    email: string;
}

// Create a function to get a user by ID
function getUserById(id: number): User | null {
    // SECOND AGENT: [MISSING CONTEXT] - This should fetch user data from a database or API
    const users: User[] = [
        { id: 1, name: 'John Doe', email: 'john.doe@example.com' },
        { id: 2, name: 'Jane Smith', email: 'jane.smith@example.com' }
    ];

    return users.find(user => user.id === id) || null;
}

// Create a function to display user information
function displayUser(user: User): void {
    console.log(`User ID: ${user.id}`);
    console.log(`Name: ${user.name}`);
    console.log(`Email: ${user.email}`);
}

// Example usage
const user = getUserById(1);
if (user) {
    displayUser(user);
} else {
    console.log('User not found');
}