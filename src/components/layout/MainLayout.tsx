import { Outlet } from 'react-router-dom';
import { Navbar } from '@Components/navbar';
import { navlist } from '@Components/navbar/itemList';
import { PropsWithChildren } from 'react';
import { Container } from '@mantine/core';

const MainLayout = ({ children }: PropsWithChildren) => {
  return (
    <div style={{ height: '100vh', display: 'flex' }}>
      <Navbar navList={navlist} />
      <Container fluid sx={{ width: '100%', backgroundColor: '#edeff3' }}>
        {children || <Outlet />}
      </Container>
    </div>
  );
};

export default MainLayout;
