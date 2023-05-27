import MainLayout from '@Components/layout/MainLayout';
import { MantineProvider } from '@mantine/core';
import { MemoryRouter } from 'react-router-dom';
import AppConfigProvider from './config/ConfigContext';
import Routes from './routes/AppRoutes';
import { useAppConfig } from './config/ConfigContext';

const App = () => {
  const { initPath } = useAppConfig();
  return (
    <AppConfigProvider>
      <MantineProvider
        theme={{ fontFamily: 'Prompt, sans-serif' }}
        withGlobalStyles
        withNormalizeCSS
      >
        <MemoryRouter initialEntries={[initPath]}>
          <MainLayout>
            <Routes />
          </MainLayout>
        </MemoryRouter>
      </MantineProvider>
    </AppConfigProvider>
  );
};

export default App;
