import React from 'react';
import ReactDOM from 'react-dom';
import 'whatwg-fetch';

class IndexPage extends React.Component {
    constructor(props){
        super(props);

        this.state = {
            users: [],
            stagings: [],
        };
    }

    componentDidMount(){
        fetch('/')
            //todo set content-type on backend
            .then(r => r.json())
            .then(r => {
                this.setState({
                    users: r.users,
                    stagings: r.stagings
                });
            })
    }

    render() {
        return (
            <div className="container">
                <div className="users">
                    {this.state.users.map(user =>
                        <div key={user.id}>{user.name}</div>
                    )}
                </div>
                <div className="stagings">
                    {this.state.stagings.map(staging =>
                        <div key={staging.id}>{staging.name}</div>
                    )}
                </div>
            </div>
        );
    }
}

ReactDOM.render(<IndexPage />, document.getElementById('root'));
