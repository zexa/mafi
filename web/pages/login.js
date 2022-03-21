import { Text } from '@nextui-org/react';
import { Input } from '@nextui-org/react';
import { Spacer } from '@nextui-org/react';
import { Button } from '@nextui-org/react';

function Login() {
    return (
        <div>
           <Text h1>Login</Text>
           <Input label="Name" placeholder="Augustinas" />
           <Spacer y={1}/>
           <Button>Submit</Button>
        </div>
    );
}

export default Login
