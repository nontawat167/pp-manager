import MainLayout from '@Components/layout/MainLayout';
import { MantineProvider } from '@mantine/core';
import { MemoryRouter } from 'react-router-dom';

import { navlist } from '@Components/navbar/itemList';
import AppConfigProvider from './config/ConfigContext';
import Routes from './routes/AppRoutes';

const App = () => {
  return (
    <AppConfigProvider>
      <MantineProvider
        theme={{ fontFamily: 'Prompt, sans-serif' }}
        withGlobalStyles
        withNormalizeCSS
      >
        <MemoryRouter initialEntries={[navlist[0].path]}>
          <MainLayout>
            <Routes />
          </MainLayout>
        </MemoryRouter>
      </MantineProvider>
    </AppConfigProvider>
  );
};

export default App;
