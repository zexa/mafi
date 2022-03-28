import { Text } from '@nextui-org/react';
import { Input } from '@nextui-org/react';
import { Spacer } from '@nextui-org/react';
import { Button } from '@nextui-org/react';
import { Link } from '@nextui-org/react';
import MafiClient from '../clients/mafi_client.ts';
import getConfig from 'next/config';
import cookieCutter from 'cookie-cutter';
import Router from 'next/router';

export async function getServerSideProps(context) {
    const mafiClient = new MafiClient(getConfig().serverRuntimeConfig.apiUrl);
    if (typeof context.query.name === 'string') {
        console.log('creating user', context.query.name);
        const user = await mafiClient.registerUser(context.query.name);
        console.log('user created', user);

        return {
            props: {
                secret: user.secret,
            },
        };
    }

    return {props: {}};
}

function Login(props) {
    if (typeof props.secret === 'string') {
        cookieCutter.set('SESSION', `${props.secret}`);
        Router.push('/lobby');
    }

    const clicky = function() {
        const name = document.getElementById('name').value;
        Router.push(`?name=${name}`);
    }

    return (
        <div>
           <Text h1>Login</Text>
           <Input id="name" label="Name" placeholder="Augustinas" />
           <Spacer y={1}/>
           <Button onClick={clicky}>Submit</Button>
        </div>
    );
}

export default Login
